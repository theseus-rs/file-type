use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63098174: FileFormat = FileFormat {
    id: 63_098_174,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio XML Drawing, version 2003-2010",
    extensions: &["vdx"],
    media_types: &["application/vnd.visio"],
    signatures: &[],
    related_formats: &[],
};
