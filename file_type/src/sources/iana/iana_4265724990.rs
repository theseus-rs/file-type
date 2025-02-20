use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4265724990: FileType = FileType {
    file_format: &FileFormat {
        id: 4_265_724_990,
        source_type: SourceType::Iana,
        name: "vnd.gentoo.ebuild",
        extensions: &[],
        media_types: &["application/vnd.gentoo.ebuild"],
        signatures: &[],
        related_formats: &[],
    },
};
