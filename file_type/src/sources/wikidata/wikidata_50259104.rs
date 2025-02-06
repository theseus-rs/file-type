use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50259104: FileFormat = FileFormat {
    id: 50_259_104,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Template, version 2013",
    extensions: &["vstx"],
    media_types: &["application/vnd.ms-visio.template.main+xml"],
    signatures: &[],
    related_formats: &[],
};
