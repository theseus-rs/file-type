use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2269888961: FileType = FileType {
    file_format: &FileFormat {
        id: 2_269_888_961,
        source_type: SourceType::Iana,
        name: "vnd.dvb.iptv.alfec-base",
        extensions: &[],
        media_types: &["application/vnd.dvb.iptv.alfec-base"],
        signatures: &[],
        related_formats: &[],
    },
};
