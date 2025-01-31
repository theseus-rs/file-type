use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125692441: FileFormat = FileFormat {
    id: 125_692_441,
    puid: "wikidata/125692441",
    name: "Microsoft PowerPoint Presentation Template",
    extensions: &["potx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
