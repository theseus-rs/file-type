use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850401: FileFormat = FileFormat {
    id: 105_850_401,
    source_type: SourceType::Wikidata,
    name: "Poser character rigging",
    extensions: &["cr2"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
