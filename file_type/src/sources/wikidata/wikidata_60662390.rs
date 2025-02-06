use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60662390: FileFormat = FileFormat {
    id: 60_662_390,
    source_type: SourceType::Wikidata,
    name: "Inkwriter Template",
    extensions: &["pdt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
