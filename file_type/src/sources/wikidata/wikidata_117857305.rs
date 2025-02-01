use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117857305: FileFormat = FileFormat {
    id: 117_857_305,
    puid: "wikidata/117857305",
    name: "Ricoh DX-1 Adapter/Fax Card file",
    extensions: &["ric"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
