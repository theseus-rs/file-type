use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272315: FileFormat = FileFormat {
    id: 111_272_315,
    puid: "wikidata/111272315",
    name: "Ensoniq SQ80 instrument file",
    extensions: &["efs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
