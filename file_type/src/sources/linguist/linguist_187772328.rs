use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_187772328: FileFormat = FileFormat {
    id: 187_772_328,
    source_type: SourceType::Linguist,
    name: "Altium Designer",
    extensions: &["OutJob", "PcbDoc", "PrjPCB", "SchDoc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
