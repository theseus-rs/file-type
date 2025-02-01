use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650305: FileFormat = FileFormat {
    id: 29_650_305,
    puid: "wikidata/29650305",
    name: "PSI-MI XML",
    extensions: &["dag", "def"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
