use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27995538: FileFormat = FileFormat {
    id: 27_995_538,
    source_type: SourceType::Wikidata,
    name: "Borderlands save file",
    extensions: &["sav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
