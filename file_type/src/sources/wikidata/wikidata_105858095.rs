use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858095: FileFormat = FileFormat {
    id: 105_858_095,
    puid: "wikidata/105858095",
    name: "Inno Setup Script (with rem)",
    extensions: &["iss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
