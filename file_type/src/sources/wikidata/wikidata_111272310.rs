use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272310: FileFormat = FileFormat {
    id: 111_272_310,
    puid: "wikidata/111272310",
    name: "Ensoniq SQ1/SQ2/KS32 instrument file",
    extensions: &["efq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
