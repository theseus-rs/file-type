use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50259222: FileFormat = FileFormat {
    id: 50_259_222,
    puid: "wikidata/50259222",
    name: "Microsoft Visio Macro-Enabled Drawing, version 2013",
    extensions: &["vsdm"],
    media_types: &["application/vnd.ms-visio.drawing.macroEnabled.main+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
