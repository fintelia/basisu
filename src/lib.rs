use basisu_sys::*;
use lazy_static::lazy_static;
use std::convert::TryInto;
use std::mem;
use std::sync::Once;

static INIT: Once = Once::new();

lazy_static! {
    static ref CODEBOOK: basist::etc1_global_selector_codebook = unsafe {
        let mut cb: basist::etc1_global_selector_codebook = mem::zeroed();
        basist::etc1_global_selector_codebook_init(
            &mut cb as *mut _,
            basist::g_global_selector_cb_size,
            &basist::g_global_selector_cb as *const _,
        );
        cb
    };
}

pub enum BasisError {
	/// The basis file is corrupt.
    InvalidFileContents,
	/// An invalid argument was provided.
    InvalidArgument,
}

#[repr(i32)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[derive(Copy, Clone)]
/// GPU texture format that basis files can be transcoded to.
pub enum OutputFormat {
    // BC formats
    /// Opaque only, no punchthrough alpha support yet, transcodes alpha slice if
    /// cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is specified
    BC1_RGB = basist::transcoder_texture_format_cTFBC1_RGB,
    /// Opaque+alpha, BC4 followed by a BC1 block, alpha channel will be opaque for opaque .basis
    /// files
    BC3_RGBA = basist::transcoder_texture_format_cTFBC3_RGBA,
    /// Red only, alpha slice is transcoded to output if
    /// cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is specified
    BC4_R = basist::transcoder_texture_format_cTFBC4_R,
    /// XY: Two BC4 blocks, X=R and Y=Alpha, .basis file should have alpha data (if not Y will be
    /// all 255's)
    BC5_RG = basist::transcoder_texture_format_cTFBC5_RG,
    /// RGB or RGBA, mode 5 for ETC1S, modes (1,2,3,5,6,7) for UASTC
    BC7_RGBA = basist::transcoder_texture_format_cTFBC7_RGBA,

    // ETC1-2 formats
    /// Opaque only, returns RGB or alpha data if cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag
    /// is specified
    ETC1_RGB = basist::transcoder_texture_format_cTFETC1_RGB,
    /// Opaque+alpha, ETC2_EAC_A8 block followed by a ETC1 block, alpha channel will be opaque for
    /// opaque .basis files
    ETC2_RGBA = basist::transcoder_texture_format_cTFETC2_RGBA,
    /// R only (ETC2 EAC R11 unsigned)
    ETC2_EAC_R11 = basist::transcoder_texture_format_cTFETC2_EAC_R11,
    /// RG only (ETC2 EAC RG11 unsigned), R=opaque.r, G=alpha - for tangent space normal maps
    ETC2_EAC_RG11 = basist::transcoder_texture_format_cTFETC2_EAC_RG11,

    /// Opaque+alpha, ASTC 4x4, alpha channel will be opaque for opaque .basis files. Transcoder uses
    /// RGB/RGBA/L/LA modes, void extent, and up to two ([0,47] and [0,255]) endpoint precisions.
    ASTC_4x4_RGBA = basist::transcoder_texture_format_cTFASTC_4x4_RGBA,

    // PVRTC formats
    /// Opaque only, RGB or alpha if cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is
    /// specified, nearly lowest quality of any texture format.
    PVRTC1_4_RGB = basist::transcoder_texture_format_cTFPVRTC1_4_RGB,
    /// Opaque+alpha, most useful for simple opacity maps. If .basis file doesn't have alpha
    /// cTFPVRTC1_4_RGB will be used instead. Lowest quality of any supported texture format.
    PVRTC1_4_RGBA = basist::transcoder_texture_format_cTFPVRTC1_4_RGBA,
    /// Opaque-only, almost BC1 quality, much faster to transcode and supports arbitrary texture
    /// dimensions (unlike PVRTC1 RGB).
    PVRTC2_4_RGB = basist::transcoder_texture_format_cTFPVRTC2_4_RGB,
    /// Opaque+alpha, slower to encode than cTFPVRTC2_4_RGB. Premultiplied alpha is highly
    /// recommended, otherwise the color channel can leak into the alpha channel on transparent
    /// blocks.
    PVRTC2_4_RGBA = basist::transcoder_texture_format_cTFPVRTC2_4_RGBA,

    // Misc compressed formats
    /// Opaque, RGB or alpha if cDecodeFlagsTranscodeAlphaDataToOpaqueFormats flag is specified. ATI
    /// ATC (GL_ATC_RGB_AMD)
    ATC_RGB = basist::transcoder_texture_format_cTFATC_RGB,
    /// Opaque+alpha, alpha channel will be opaque for opaque .basis files. ATI ATC
    /// (GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD)
    ATC_RGBA = basist::transcoder_texture_format_cTFATC_RGBA,
    /// Opaque only, uses exclusively CC_MIXED blocks. Notable for having a 8x4 block
    /// size. GL_3DFX_texture_compression_FXT1 is supported on Intel integrated GPU's (such as HD
    /// 630).
    FXT1_RGB = basist::transcoder_texture_format_cTFFXT1_RGB,

    // Uncompressed formats
    /// 32bpp RGBA image stored in raster (not block) order in memory, R is first byte, A is last
    /// byte.
    RGBA32 = basist::transcoder_texture_format_cTFRGBA32,
    /// 16bpp RGB image stored in raster (not block) order in memory, R at bit position 11
    RGB565 = basist::transcoder_texture_format_cTFRGB565,
    /// 16bpp RGBA image stored in raster (not block) order in memory, R at bit position 12, A at
    /// bit position 0
    RGBA4444 = basist::transcoder_texture_format_cTFRGBA4444,
    /// 16bpp RGB image stored in raster (not block) order in memory, R at bit position 0
    BGR565 = basist::transcoder_texture_format_cTFBGR565,
}
impl OutputFormat {
    pub fn bytes_per_block(&self) -> u32 {
        unsafe {
            basist::basis_get_bytes_per_block_or_pixel(*self as basist::transcoder_texture_format)
        }
    }
    pub fn block_width(&self) -> u32 {
        if unsafe {
            basist::basis_block_format_is_uncompressed(*self as basist::transcoder_texture_format)
        } {
            return 1;
        }
        unsafe { basist::basis_get_block_width(*self as basist::transcoder_texture_format) }
    }
    pub fn block_height(&self) -> u32 {
        if unsafe {
            basist::basis_block_format_is_uncompressed(*self as basist::transcoder_texture_format)
        } {
            return 1;
        }
        unsafe { basist::basis_get_block_height(*self as basist::transcoder_texture_format) }
    }
}

/// 
pub struct BasisTranscoder(basist::basisu_transcoder);

pub struct BasisFileTranscoder<'a> {
    transcoder: &'a mut BasisTranscoder,
    data: &'a [u8],
}

impl BasisTranscoder {
	/// Create a new transcoder. The first time this is called, it does some library wide
	/// initialization.
    pub fn new() -> Self {
		INIT.call_once(|| unsafe {
			basist::basisu_transcoder_init();
		});

        unsafe {
            let mut t: basist::basisu_transcoder = mem::zeroed();
            basist::basisu_transcoder_basisu_transcoder(&mut t as *mut _, &*CODEBOOK as *const _);
            Self(t)
        }
    }

	/// Return whether the file checksums are valid. This is an expensive operation because it must
	/// scan the full file data.
    pub fn validate_file_checksums(&self, data: &[u8], full_validation: bool) -> bool {
        unsafe {
            self.0.validate_file_checksums(
                data.as_ptr() as *const _,
                data.len().try_into().unwrap(),
                full_validation,
            )
        }
    }

	/// Check just whether the header indicates a valid .basis file.
    pub fn validate_file_header(&self, data: &[u8]) -> bool {
        unsafe {
            self.0
                .validate_header(data.as_ptr() as *const _, data.len().try_into().unwrap())
        }
    }

	/// Return the number of images in the provided .basis file.
    pub fn get_total_images(&self, data: &[u8]) -> u32 {
        unsafe {
            self.0
                .get_total_images(data.as_ptr() as *const _, data.len().try_into().unwrap())
        }
    }

	/// Return the number of levels in the indicated image of the provided .basis file.
    pub fn get_total_image_levels(&self, data: &[u8], image_index: u32) -> u32 {
        unsafe {
            self.0.get_total_image_levels(
                data.as_ptr() as *const _,
                data.len().try_into().unwrap(),
                image_index,
            )
        }
    }

	/// Initialize the transcoder to begin transcoding this .basis file.
    pub fn start_transcoding<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Result<BasisFileTranscoder<'a>, BasisError> {
        unsafe {
            if !self
                .0
                .start_transcoding(data.as_ptr() as *const _, data.len().try_into().unwrap())
            {
                return Err(BasisError::InvalidFileContents);
            }
        }

        Ok(BasisFileTranscoder {
            transcoder: self,
            data,
        })
    }
}

impl<'a> BasisFileTranscoder<'a> {
	/// Return the total number of images.
    pub fn get_total_images(&self) -> u32 {
		self.transcoder.get_total_images(self.data)
    }
	/// Return the total number of levels in the image with index `image_index`.
    pub fn get_total_image_levels(&self, image_index: u32) -> u32 {
		self.transcoder.get_total_image_levels(self.data, image_index)
	}

	/// Return the dimension of the indicated `image_index` / `level_index` pair.
    pub fn level_dimensions(
        &self,
        image_index: u32,
        level_index: u32,
    ) -> Result<(u32, u32), BasisError> {
        let mut level_info: basist::basisu_image_level_info = unsafe { mem::zeroed() };
        unsafe {
            if !self.transcoder.0.get_image_level_info(
                self.data.as_ptr() as *const _,
                self.data.len() as u32,
                &mut level_info as *mut _,
                image_index,
                level_index,
            ) {
                return Err(BasisError::InvalidArgument);
            }
        }

        Ok((level_info.m_width, level_info.m_height))
    }

	/// Transcode the indicated `image_index` / `level_index` pair into the provided output
	/// buffer. The resulting data will be in format `output_format`.
    pub fn transcode_image_level(
        &self,
        image_index: u32,
        level_index: u32,
        output: &mut [u8],
        output_format: OutputFormat,
    ) -> Result<(), BasisError> {
        let output_size_blocks = (output.len() / output_format.bytes_per_block() as usize)
            .try_into()
            .unwrap();

        unsafe {
            if !self.transcoder.0.transcode_image_level(
                self.data.as_ptr() as *const _,
                self.data.len() as u32,
                image_index,
                level_index,
                output.as_mut_ptr() as *mut _,
                output_size_blocks,
                output_format as basist::transcoder_texture_format,
                0,
                0,
                std::ptr::null_mut(),
                0,
            ) {
                return Err(BasisError::InvalidFileContents);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
