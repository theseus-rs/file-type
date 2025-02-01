use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50259104: FileFormat = FileFormat {
    id: 50_259_104,
    puid: "wikidata/50259104",
    name: "Microsoft Visio Template, version 2013",
    extensions: &["vstx"],
    media_types: &["application/vnd.ms-visio.template.main+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
