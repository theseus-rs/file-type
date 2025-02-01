use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111262994: FileFormat = FileFormat {
    id: 111_262_994,
    puid: "wikidata/111262994",
    name: "Aureal 'Aspen' bank file",
    extensions: &["arl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
