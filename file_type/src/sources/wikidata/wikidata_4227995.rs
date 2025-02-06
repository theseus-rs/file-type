use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4227995: FileFormat = FileFormat {
    id: 4_227_995,
    source_type: SourceType::Wikidata,
    name: "eMule collection",
    extensions: &["emulecollection"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
