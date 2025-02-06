use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130395727: FileFormat = FileFormat {
    id: 130_395_727,
    source_type: SourceType::Wikidata,
    name: "ODIN file format",
    extensions: &["odin"],
    media_types: &["text/odin"],
    signatures: &[],
    related_formats: &[],
};
