use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122150082: FileFormat = FileFormat {
    id: 122_150_082,
    puid: "wikidata/122150082",
    name: "TaxAct 2008 Tax Return File",
    extensions: &["ta8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
