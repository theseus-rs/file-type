use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63098174: FileFormat = FileFormat {
    id: 63_098_174,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio XML Drawing, version 2003-2010",
    extensions: &["vdx"],
    media_types: &["application/vnd.visio"],
    internal_signatures: &[],
    related_formats: &[],
};
