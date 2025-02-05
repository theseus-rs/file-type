use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131426714: FileFormat = FileFormat {
    id: 131_426_714,
    source_type: SourceType::Wikidata,
    name: "X++ source code file format",
    extensions: &["xpp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
