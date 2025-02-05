use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2269888961: FileFormat = FileFormat {
    id: 2_269_888_961,
    source_type: SourceType::Iana,
    name: "vnd.dvb.iptv.alfec-base",
    extensions: &[],
    media_types: &["application/vnd.dvb.iptv.alfec-base"],
    signatures: &[],
    related_formats: &[],
};
