use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117851210: FileFormat = FileFormat {
    id: 117_851_210,
    puid: "wikidata/117851210",
    name: "Tektronix Plot 10 file",
    extensions: &["p10"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
