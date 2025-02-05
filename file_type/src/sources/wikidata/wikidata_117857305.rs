use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117857305: FileFormat = FileFormat {
    id: 117_857_305,
    source_type: SourceType::Wikidata,
    name: "Ricoh DX-1 Adapter/Fax Card file",
    extensions: &["ric"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
