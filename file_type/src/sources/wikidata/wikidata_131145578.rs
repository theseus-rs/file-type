use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131145578: FileFormat = FileFormat {
    id: 131_145_578,
    source_type: SourceType::Wikidata,
    name: "Spice source file format",
    extensions: &["spice"],
    media_types: &["text/x-spice"],
    signatures: &[],
    related_formats: &[],
};
