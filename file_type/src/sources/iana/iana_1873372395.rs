use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1873372395: FileType = FileType {
    file_format: &FileFormat {
        id: 1_873_372_395,
        source_type: SourceType::Iana,
        name: "vnd.ibm.afplinedata (OBSOLETED in favor of vnd.afpc.afplinedata)",
        extensions: &[],
        media_types: &["application/vnd.ibm.afplinedata"],
        signatures: &[],
        related_formats: &[],
    },
};
