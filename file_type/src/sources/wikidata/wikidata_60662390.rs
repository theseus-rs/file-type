use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60662390: FileFormat = FileFormat {
    id: 60_662_390,
    source_type: SourceType::Wikidata,
    name: "Inkwriter Template",
    extensions: &["pdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
