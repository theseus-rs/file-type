use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757993: FileFormat = FileFormat {
    id: 28_757_993,
    puid: "wikidata/28757993",
    name: "IWA",
    extensions: &["iwa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
