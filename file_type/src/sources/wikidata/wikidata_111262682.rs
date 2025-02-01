use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111262682: FileFormat = FileFormat {
    id: 111_262_682,
    puid: "wikidata/111262682",
    name: "Yamaha A3000 sample file",
    extensions: &["a3s"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
