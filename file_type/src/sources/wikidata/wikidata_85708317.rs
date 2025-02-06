use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85708317: FileFormat = FileFormat {
    id: 85_708_317,
    source_type: SourceType::Wikidata,
    name: "Calendar Creator File",
    extensions: &["cc3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
