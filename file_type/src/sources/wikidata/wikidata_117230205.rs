use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117230205: FileFormat = FileFormat {
    id: 117_230_205,
    source_type: SourceType::Wikidata,
    name: "PostScript file",
    extensions: &["ps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
