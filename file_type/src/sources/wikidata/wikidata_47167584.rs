use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47167584: FileFormat = FileFormat {
    id: 47_167_584,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Painting file format",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
