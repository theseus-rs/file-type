use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130478829: FileFormat = FileFormat {
    id: 130_478_829,
    source_type: SourceType::Wikidata,
    name: "pkg-config file format",
    extensions: &["pc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
