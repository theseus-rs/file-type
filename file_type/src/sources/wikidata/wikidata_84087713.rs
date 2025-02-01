use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_84087713: FileFormat = FileFormat {
    id: 84_087_713,
    puid: "wikidata/84087713",
    name: "RootsMagic Database",
    extensions: &["rmgc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
