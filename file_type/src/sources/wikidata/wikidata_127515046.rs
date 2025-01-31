use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127515046: FileFormat = FileFormat {
    id: 127_515_046,
    puid: "wikidata/127515046",
    name: "Typescript declaration file",
    extensions: &["d.ts"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
