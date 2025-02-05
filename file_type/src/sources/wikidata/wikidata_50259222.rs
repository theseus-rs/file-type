use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50259222: FileFormat = FileFormat {
    id: 50_259_222,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Macro-Enabled Drawing, version 2013",
    extensions: &["vsdm"],
    media_types: &["application/vnd.ms-visio.drawing.macroEnabled.main+xml"],
    signatures: &[],
    related_formats: &[],
};
