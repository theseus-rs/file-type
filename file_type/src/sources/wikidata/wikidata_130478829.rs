use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130478829: FileFormat = FileFormat {
    id: 130_478_829,
    source_type: SourceType::Wikidata,
    name: "pkg-config file format",
    extensions: &["pc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
