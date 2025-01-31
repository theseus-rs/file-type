use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63095276: FileFormat = FileFormat {
    id: 63_095_276,
    puid: "wikidata/63095276",
    name: "Microsoft Powerpoint for Windows file format",
    extensions: &["pptx"],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
