use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1546911: FileFormat = FileFormat {
    id: 1_546_911,
    source_type: SourceType::Wikidata,
    name: "Cross-Platform Installer Module",
    extensions: &["xpi"],
    media_types: &["application/x-xpinstall"],
    internal_signatures: &[],
    related_formats: &[],
};
