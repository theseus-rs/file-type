use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87984761: FileFormat = FileFormat {
    id: 87_984_761,
    source_type: SourceType::Wikidata,
    name: "CorelCHART Document 3-4",
    extensions: &["cch"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
