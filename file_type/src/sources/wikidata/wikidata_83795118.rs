use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83795118: FileFormat = FileFormat {
    id: 83_795_118,
    puid: "wikidata/83795118",
    name: "ZFO (Message) File",
    extensions: &["zfo"],
    media_types: &["application/vnd.software602.filler.form-xml-zip"],
    internal_signatures: &[],
    related_formats: &[],
};
