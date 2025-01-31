use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122403904: FileFormat = FileFormat {
    id: 122_403_904,
    puid: "wikidata/122403904",
    name: "CodeWarrior Constructor Resource File",
    extensions: &["rsr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
