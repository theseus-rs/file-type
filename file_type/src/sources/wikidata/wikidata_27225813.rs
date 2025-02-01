use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27225813: FileFormat = FileFormat {
    id: 27_225_813,
    puid: "wikidata/27225813",
    name: "OpenDocument Database, version 1.2",
    extensions: &["odb"],
    media_types: &["application/vnd.oasis.opendocument.database"],
    internal_signatures: &[],
    related_formats: &[],
};
