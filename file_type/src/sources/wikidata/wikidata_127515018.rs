use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127515018: FileFormat = FileFormat {
    id: 127_515_018,
    puid: "wikidata/127515018",
    name: "Typescript implementation file",
    extensions: &["ts", "ts"],
    media_types: &["application/x-typescript", "text/x-typescript"],
    internal_signatures: &[],
    related_formats: &[],
};
