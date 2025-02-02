use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117857305: FileFormat = FileFormat {
    id: 117_857_305,
    source_type: SourceType::Wikidata,
    name: "Ricoh DX-1 Adapter/Fax Card file",
    extensions: &["ric"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
