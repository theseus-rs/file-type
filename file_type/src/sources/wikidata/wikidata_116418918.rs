use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116418918: FileFormat = FileFormat {
    id: 116_418_918,
    puid: "wikidata/116418918",
    name: "Adobe Photoshop Color Table",
    extensions: &["act"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
