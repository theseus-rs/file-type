use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117485972: FileFormat = FileFormat {
    id: 117_485_972,
    puid: "wikidata/117485972",
    name: "Audacity Project File 3.x",
    extensions: &["aup3"],
    media_types: &["application/x-audacity-project+sqlite3"],
    internal_signatures: &[],
    related_formats: &[],
};
