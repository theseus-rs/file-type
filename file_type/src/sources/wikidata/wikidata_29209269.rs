use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29209269: FileFormat = FileFormat {
    id: 29_209_269,
    source_type: SourceType::Wikidata,
    name: "Z",
    extensions: &["z"],
    media_types: &["application/x-compress"],
    internal_signatures: &[],
    related_formats: &[],
};
