use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63095276: FileFormat = FileFormat {
    id: 63_095_276,
    source_type: SourceType::Wikidata,
    name: "Microsoft Powerpoint for Windows file format",
    extensions: &["pptx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
    signatures: &[],
    related_formats: &[],
};
