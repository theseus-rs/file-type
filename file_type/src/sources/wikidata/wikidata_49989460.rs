use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_49989460: FileFormat = FileFormat {
    id: 49_989_460,
    source_type: SourceType::Wikidata,
    name: "ActiveX License Package file",
    extensions: &["lpk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
