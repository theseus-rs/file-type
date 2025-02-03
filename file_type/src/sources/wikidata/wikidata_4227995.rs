use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4227995: FileFormat = FileFormat {
    id: 4_227_995,
    source_type: SourceType::Wikidata,
    name: "eMule collection",
    extensions: &["emulecollection"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
