use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28049655: FileFormat = FileFormat {
    id: 28_049_655,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Device-Independent Binary Plotter File",
    extensions: &["adi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
