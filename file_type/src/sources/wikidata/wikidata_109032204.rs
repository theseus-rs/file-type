use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109032204: FileFormat = FileFormat {
    id: 109_032_204,
    source_type: SourceType::Wikidata,
    name: "Zeiss Vision Image",
    extensions: &["zvi"],
    media_types: &["image/zvi"],
    internal_signatures: &[],
    related_formats: &[],
};
