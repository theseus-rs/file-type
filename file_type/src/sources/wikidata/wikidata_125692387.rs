use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125692387: FileFormat = FileFormat {
    id: 125_692_387,
    puid: "wikidata/125692387",
    name: "Microsoft PowerPoint 2007 XML Template",
    extensions: &["potm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
