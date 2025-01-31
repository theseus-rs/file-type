use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112653501: FileFormat = FileFormat {
    id: 112_653_501,
    puid: "wikidata/112653501",
    name: "Professional Draw 1.0 backup file",
    extensions: &["pd~"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
