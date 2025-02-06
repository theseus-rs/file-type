use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111440679: FileFormat = FileFormat {
    id: 111_440_679,
    source_type: SourceType::Wikidata,
    name: "Perl POD File",
    extensions: &["pod"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
